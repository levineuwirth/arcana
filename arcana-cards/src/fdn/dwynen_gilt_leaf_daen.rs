//! Dwynen, Gilt-Leaf Daen — `{2}{G}{G}` 3/4 Legendary Elf Warrior with Reach.
//!
//! "Reach
//!  Other Elf creatures you control get +1/+1.
//!  Whenever Dwynen attacks, you gain 1 life for each attacking Elf you
//!  control."
//!
//! Reach is a base keyword. The "Other Elf creatures you control get +1/+1"
//! line is a pure static continuous anthem with no triggered/activated hook on
//! this card shape — GAP'd. The attack trigger gains life equal to the number
//! of attacking Elves you control (dynamic; computed at resolution).

// GAP (static): "Other Elf creatures you control get +1/+1." — pure static
// continuous anthem, not a triggered/activated ability.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwynen, Gilt-Leaf Daen");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: gain_life_per_attacking_elf,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_life_per_attacking_elf(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Elf")
        .controlled_by(ControllerConstraint::You)
        .attacking_only();
    let n = script::count_matching(state, &filter, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
