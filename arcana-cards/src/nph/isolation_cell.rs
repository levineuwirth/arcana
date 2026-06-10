//! Isolation Cell — `{4}` artifact (Prison Realm flavor; Battlebond/War
//! era). "Whenever an opponent casts a creature spell, that player loses
//! 2 life unless they pay {2}." SpellCast(creature, Opponent) trigger;
//! "that player" is read as the opponent (exact in two-player games);
//! the unless-pays gate is an OptionalPayment with the punishment in
//! `else_effect`.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Isolation Cell");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::creature()),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: tax_creature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn tax_creature_cast(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "That player" is the casting opponent — exact in two-player games.
    let opponents = script::opponents(state, trig.controller);
    let Some(&p) = opponents.first() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: p,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{2}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::LoseLife { player: p, amount: 2 })),
    }]
}
