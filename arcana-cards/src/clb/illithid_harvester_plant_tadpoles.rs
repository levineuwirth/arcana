//! Illithid Harvester // Plant Tadpoles — `{4}{U}` 4/4 blue Horror (Creature with Adventure).
//! Ceremorphosis ETB: turn any number of target tapped nontoken creatures face down (2/2 Horrors).
//! Adventure — Plant Tadpoles `{X}{U}{U}`: Tap X target creatures; they don't untap next untap step.
//! GAP: "turn creatures face down as 2/2 Horrors" — no Effect variant for face-down transforms.
//! GAP: "they don't untap during their controllers' next untap step" — no ongoing untap prevention.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illithid Harvester");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let adv_name = reg.interner_mut().intern("Plant Tadpoles");

    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{X}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Tap X target creatures. They don't untap during their controllers' next untap steps.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::Any,
            controller: None,
        }],
        modal: None,
        effect: plant_tadpoles_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: ceremorphosis_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .tapped_only()
                            .nontoken()
                            .controlled_by(ControllerConstraint::Any),
                    ),
                    count: TargetCount::Any,
                    controller: None,
                }],
            })
            .with_adventure(adventure),
    )
}

fn ceremorphosis_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "turn any number of target tapped nontoken creatures face down as 2/2 Horrors" —
    // no Effect variant for face-down transforms with type change.
    Vec::new()
}

fn plant_tadpoles_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Tap all chosen targets.
    // GAP: "they don't untap during controllers' next untap step" — no ongoing untap prevention.
    entry.targets.targets.iter()
        .filter_map(|t| if let TargetChoice::Object(id) = t { Some(*id) } else { None })
        .map(|id| Effect::Tap { target: id })
        .collect()
}
