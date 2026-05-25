//! Ardent Electromancer — `{2}{R}` 3/2 red Human Wizard.
//! "When this creature enters, add {R} for each creature in your party."
//! (Your party consists of up to one each of Cleric, Rogue, Warrior, and
//! Wizard.)
//!
//! GAP: "party" count (up to 4, one per class) requires per-subtype
//! logic across four types. Using script::count_matching for each class
//! and clamping to 1 per class.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ardent Electromancer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _cleric = reg.interner_mut().intern("Cleric");
    let _rogue = reg.interner_mut().intern("Rogue");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_add_mana_for_party,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_add_mana_for_party(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cleric_f = script::subtype_filter(reg, "Cleric").controlled_by(ControllerConstraint::You);
    let rogue_f = script::subtype_filter(reg, "Rogue").controlled_by(ControllerConstraint::You);
    let warrior_f = script::subtype_filter(reg, "Warrior").controlled_by(ControllerConstraint::You);
    let wizard_f = script::subtype_filter(reg, "Wizard").controlled_by(ControllerConstraint::You);
    let mut party = 0u32;
    if script::count_matching(state, &cleric_f, trig.controller) > 0 { party += 1; }
    if script::count_matching(state, &rogue_f, trig.controller) > 0 { party += 1; }
    if script::count_matching(state, &warrior_f, trig.controller) > 0 { party += 1; }
    if script::count_matching(state, &wizard_f, trig.controller) > 0 { party += 1; }
    if party == 0 {
        return Vec::new();
    }
    let mana = vec![ManaUnit::plain(ManaColor::Red, trig.source); party as usize];
    vec![Effect::AddMana {
        player: trig.controller,
        mana,
    }]
}
