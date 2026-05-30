//! Kytheon, Hero of Akros // Gideon, Battle-Forged — `{W}` Legendary Human Soldier 2/1.
//!
//! Front face (Kytheon):
//! - At end of combat, if Kytheon and at least two other creatures attacked this combat,
//!   exile Kytheon, then return him to the battlefield transformed (Gideon planeswalker).
//!   GAP: "if Kytheon and at least two other creatures attacked this combat" — attack-count
//!   condition not expressible; wiring on EndCombat step without the guard.
//! - {2}{W}: Kytheon gains indestructible until end of turn.
//!   GAP: ActivatedAbilityDef not modeled for transform cards; omitting activated ability.
//!
//! Back face (Gideon, Battle-Forged): Legendary Planeswalker — Gideon, loyalty 3.
//! GAP: back-face-only planeswalker loyalty abilities (+2 goad, +1 indestructible+untap, 0 creature)
//! not modeled (back-face triggered abilities not auto-installed on transform).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kytheon, Hero of Akros");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Gideon, Battle-Forged — Legendary Planeswalker — Gideon, loyalty 3
    let back_name = reg.interner_mut().intern("Gideon, Battle-Forged");
    let gideon_sub = reg.interner_mut().intern("Gideon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(gideon_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(3),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front transform trigger: at end of combat step, transform.
    // GAP: "if Kytheon and at least two other creatures attacked this combat" condition
    // not expressible. Wiring on EndCombat step without the attack-count guard.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_to_gideon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn transform_to_gideon(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire if Kytheon + 2 others attacked this combat.
    // Attack-count condition is not expressible with available TriggerCondition variants.
    vec![Effect::Transform { target: trig.source }]
}
