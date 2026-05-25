//! Ardent Electromancer — `{2}{R}` 3/2 red Human Wizard.
//! "When this creature enters, add {R} for each creature in your party.
//! (Your party consists of up to one each of Cleric, Rogue, Warrior, and
//! Wizard.)"
//! GAP: "party" mechanic (up to 4 creatures of specific subtypes) is not
//! directly modeled; approximating by counting up to 4 party subtypes.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
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
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_party_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_party_mana(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Party = up to 1 Cleric, 1 Rogue, 1 Warrior, 1 Wizard you control
    let party_types = ["Cleric", "Rogue", "Warrior", "Wizard"];
    let mut count = 0u32;
    for subtype_name in &party_types {
        let filter = script::subtype_filter(reg, subtype_name)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, trig.controller) > 0 {
            count += 1;
        }
    }
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); count as usize],
    }]
}
