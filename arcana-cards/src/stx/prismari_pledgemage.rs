//! Prismari Pledgemage — `{U/R}{U/R}` 3/3 Orc Wizard with Defender.
//! "Magecraft — Whenever you cast or copy an instant or sorcery spell, this
//! creature can attack this turn as though it didn't have defender." — the
//! "can attack as though it didn't have defender" effect is not expressible
//! in the documented Effect surface, GAP. (The copy half of the trigger is
//! also not capturable; the cast half is wired as the trigger condition.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prismari Pledgemage");
    let orc = reg.interner_mut().intern("Orc");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                    TypeLine::INSTANT | TypeLine::SORCERY,
                ))),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: magecraft_attack_anyway,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn magecraft_attack_anyway(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can attack this turn as though it didn't have defender" — no
    // effect to suppress Defender for the turn.
    Vec::new()
}
