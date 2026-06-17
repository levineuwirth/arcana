//! Curie, Emergent Intelligence — `{1}{U}` 1/3 Legendary Artifact Creature — Robot.
//! Whenever Curie deals combat damage to a player, draw cards equal to its base power.
//! {1}{U}, Exile another nontoken artifact creature you control: Curie becomes
//! a copy of the exiled creature, except it has "Whenever this creature deals
//! combat damage to a player, draw cards equal to its base power."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Curie, Emergent Intelligence");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: draw_equal_to_power,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "{1}{U}, Exile another nontoken artifact creature you control:
        // Curie becomes a copy of the exiled creature ...". ActivationCost has
        // no exile-another-permanent cost field (only sacrifice_other /
        // tap_other / discard_other), and there is no self-copy effect
        // (CopyPermanent mints a separate token rather than rewriting THIS
        // object's characteristics).
    )
}

fn draw_equal_to_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity: "base power" is approximated by current power.
    let n = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
