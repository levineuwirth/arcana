//! Mathise, Surge Channeler — `{2}{U}` 1/1 Legendary Human Elf Shaman Sorcerer.
//! Whenever you cast an instant or sorcery spell with mana value 3 or greater,
//! roll a d20 (1-9 each player draws / 10-19 you draw / 20 copy that spell).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mathise, Surge Channeler");
    let human = reg.interner_mut().intern("Human");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
                            .with_min_cmc(3),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: roll_d20,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn roll_d20(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "roll a d20" with an outcome table (1-9 / 10-19 / 20) is not an
    // expressible Effect; no die-roll primitive exists in the catalog.
    Vec::new()
}
