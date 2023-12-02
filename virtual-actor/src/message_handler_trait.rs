//! Message handler trait

use std::future::Future;

use crate::Message;

use super::actor_trait::Actor;

/// Message Handler trait
/// Responsible for handling specific message type
pub trait MessageHandler<M: Message>: Actor {
    /// Handler function
    fn handle(
        &mut self,
        msg: M,
        ctx: &Self::ActorContext,
    ) -> impl Future<Output = <M as Message>::Result>;
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        actor_addr::{ActorAddr, AddrError},
        responder_trait::ResponderError,
        Actor, ActorContext, CancellationToken, Message, MessageEnvelope, MessageEnvelopeFactory,
        Responder, WeakActorRef,
    };

    use super::MessageHandler;

    struct TestMessage {
        data: u32,
    }

    impl Message for TestMessage {
        type Result = u32;
    }

    enum TestMessagesEnvelope {
        TestMessage(TestMessage, Option<Box<dyn Responder<TestMessage>>>),
    }

    impl MessageEnvelope<TestActor> for TestMessagesEnvelope {}

    impl std::fmt::Debug for TestMessagesEnvelope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestMessagesEnvelope::TestMessage(_msg, _) => {
                    f.write_str("TestMessagesEnvelope::TestMessage(")?;
                    f.write_fmt(format_args!("{}, _)", 0usize))
                }
            }
        }
    }

    impl MessageEnvelopeFactory<TestActor, TestMessage> for TestMessagesEnvelope {
        fn from_message<R: Responder<TestMessage> + Sized + 'static>(
            msg: TestMessage,
            responder: Option<R>,
        ) -> Self {
            let responder: Option<Box<dyn Responder<TestMessage>>> = match responder {
                Some(r) => Some(Box::new(r)),
                None => None,
            };
            Self::TestMessage(msg, responder)
        }
    }

    struct TestActor {
        handler_state: Rc<RefCell<u32>>,
    }

    impl Actor for TestActor {
        type MessagesEnvelope = TestMessagesEnvelope;
        type ActorContext = TestContext;

        async fn handle_envelope(
            &mut self,
            msg: Self::MessagesEnvelope,
            ctx: &Self::ActorContext,
        ) -> Result<(), ResponderError> {
            match msg {
                TestMessagesEnvelope::TestMessage(msg, responder) => {
                    let result = self.handle(msg, ctx).await;
                    if let Some(mut responder) = responder {
                        responder.respond(result)?;
                    }
                }
            }
            Ok(())
        }
    }

    impl MessageHandler<TestMessage> for TestActor {
        async fn handle(
            &mut self,
            msg: TestMessage,
            _ctx: &Self::ActorContext,
        ) -> <TestMessage as Message>::Result {
            let fut = async {
                let data = *self.handler_state.borrow();
                self.handler_state.replace(data + msg.data);
                *self.handler_state.borrow()
            };

            fut.await
        }
    }

    struct TestResponder;

    impl Responder<TestMessage> for TestResponder {
        fn respond(
            &mut self,
            result: <TestMessage as Message>::Result,
        ) -> Result<(), ResponderError> {
            assert_eq!(result, 42);
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestAddr;

    impl ActorAddr<TestActor> for TestAddr {
        type WeakRef = WeakRef;

        async fn send<M>(&self, _msg: M) -> Result<M::Result, AddrError>
        where
            M: Message,
            TestActor: MessageHandler<M>,
            TestMessagesEnvelope: MessageEnvelopeFactory<TestActor, M>,
        {
            Err(AddrError::ActorNotReady)
        }

        fn dispatch<M>(&self, _msg: M) -> Result<(), AddrError>
        where
            M: Message,
            TestActor: MessageHandler<M>,
            TestMessagesEnvelope: MessageEnvelopeFactory<TestActor, M>,
        {
            Ok(())
        }

        fn weak_ref(&self) -> Self::WeakRef {
            todo!()
        }
    }

    #[derive(Clone)]
    struct WeakRef;

    impl WeakActorRef<TestActor, TestAddr> for WeakRef {
        fn upgrade(&self) -> Option<TestAddr> {
            todo!()
        }
    }

    struct TestCancellationToken;

    impl CancellationToken for TestCancellationToken {
        async fn cancelled(&self) {
            todo!()
        }
    }

    #[derive(Clone)]
    struct TestContext;

    impl ActorContext<TestActor> for TestContext {
        type Addr = TestAddr;
        type CancellationToken = TestCancellationToken;

        fn self_addr(&self) -> &Self::Addr {
            &TestAddr
        }

        fn stop(&self) {
            todo!()
        }

        fn cancellation_token(&self) -> &Self::CancellationToken {
            &TestCancellationToken
        }
    }

    #[test]
    fn test_envelope_handler() {
        let mut handler = TestActor {
            handler_state: Rc::new(RefCell::new(0)),
        };

        futures::executor::block_on(async move {
            let ctx = TestContext;
            let messages = TestMessagesEnvelope::TestMessage(
                TestMessage { data: 42 },
                Some(Box::new(TestResponder)),
            );
            handler
                .handle_envelope(messages, &ctx)
                .await
                .expect("Failed to handle envelope");
        });
    }

    #[test]
    fn test_messages_handler() {
        let mut handler = TestActor {
            handler_state: Rc::new(RefCell::new(0)),
        };

        futures::executor::block_on(async move {
            let ctx = TestContext;
            let result = handler.handle(TestMessage { data: 42 }, &ctx).await;
            assert_eq!(result, 42);
            let result = handler.handle(TestMessage { data: 42 }, &ctx).await;
            assert_eq!(result, 84);
        });
    }
}
