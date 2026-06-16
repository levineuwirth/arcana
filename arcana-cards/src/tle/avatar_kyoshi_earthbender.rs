//! Avatar Kyoshi, Earthbender — `{5}{G}{G}{G}` 6/6 Legendary Creature —
//! Human Avatar (G).
//!
//! * During your turn, Avatar Kyoshi has hexproof. (Conditional static —
//!   GAP, not a triggered/activated ability.)
//! * At the beginning of combat on your turn, earthbend 8, then untap
//!   that land. The "earthbend N" mechanic (animate a target land into a
//!   0/0 with haste, add N +1/+1 counters, return-on-leave) has no
//!   expressible Effect variant — GAP the trigger's effect body while
//!   still emitting the combat trigger shell.
//!
//! Earthbend is not a usable keyword variant, so `keywords` is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar Kyoshi, Earthbender");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "During your turn, Avatar Kyoshi has hexproof" is a
        // conditional continuous static, not a base keyword.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: earthbend_eight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn earthbend_eight(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "earthbend 8, then untap that land" — the earthbend mechanic
    // (animate a target land into a 0/0 haste creature, add eight +1/+1
    // counters, return-to-battlefield-tapped rider) has no expressible
    // Effect variant in the demonstrated API.
    Vec::new()
}
