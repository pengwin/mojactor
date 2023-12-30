use serde::Deserialize;
use serde::Serialize;
use virtual_actor::ActorAddr;
use virtual_actor::MessageHandler;
use virtual_actor_runtime::LocalAddr;

use virtual_actor_runtime::prelude::Actor;
use virtual_actor_runtime::prelude::Message;
use virtual_actor_runtime::prelude::VirtualActor;
use virtual_actor_runtime::prelude::VirtualMessage;

#[derive(Actor, VirtualActor)]
#[message(TestMessage)]
struct TestActorWithMessages {
    id: u32,
}

impl MessageHandler<TestMessage> for TestActorWithMessages {
    async fn handle(
        &mut self,
        _msg: TestMessage,
        _ctx: &Self::ActorContext,
    ) -> <TestMessage as Message>::Result {
        Ok(10)
    }
}

#[derive(Actor, VirtualActor)]
struct TestActorWithoutMessages {
    id: u32,
}

#[test]
fn test_derive_virtual_actor_with_messages_name() {
    assert_eq!(
        TestActorWithMessages::name(),
        stringify!(TestActorWithMessages)
    );
}

#[test]
fn test_derive_virtual_actor_name() {
    assert_eq!(
        TestActorWithoutMessages::name(),
        stringify!(TestActorWithoutMessages)
    );
}

#[test]
fn test_derive_virtual_actor_id() {
    let actor = TestActorWithMessages { id: 42 };
    assert_eq!(actor.id(), &42);
}

#[derive(Clone)]
struct TestContext;

impl virtual_actor::ActorContext<TestActorWithContext> for TestContext {
    type Addr = LocalAddr<TestActorWithContext>;

    fn self_addr(&self) -> &<Self::Addr as ActorAddr<TestActorWithContext>>::WeakRef {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    type CancellationToken = TestCancellationToken;

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &TestCancellationToken
    }
}

struct TestCancellationToken;

impl virtual_actor::utils::CancellationToken for TestCancellationToken {
    async fn cancelled(&self) {
        todo!()
    }
}

#[derive(Actor, VirtualActor)]
#[context(TestContext)]
struct TestActorWithContext {
    id: u32,
}

#[test]
fn test_derive_virtual_actor_with_context_name() {
    assert_eq!(
        TestActorWithContext::name(),
        stringify!(TestActorWithContext)
    );
}

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(Result<u64, u8>)]
struct TestMessage;

#[test]
fn test_derive_virtual_message_name() {
    assert_eq!(TestMessage::name(), stringify!(TestMessage));
}

#[test]
fn test_derive_virtual_message_result() {
    let s = <TestMessage as Message>::Result::Ok(42);
    assert_eq!(s, Ok(42));
}
