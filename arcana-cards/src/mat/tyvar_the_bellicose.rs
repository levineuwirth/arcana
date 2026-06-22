//! Tyvar the Bellicose — `{2}{B}{G}` 5/4 Legendary Elf Warrior.
//! "Whenever one or more Elves you control attack, they gain deathtouch
//!  until end of turn.
//!  Each creature you control has '[mana-ability counter trigger].'" (static
//!  ability-grant — GAP'd.)
//!
//! The attack trigger is wired as a per-attacker CreatureAttacks watcher
//! filtered to Elves you control; each attacking Elf is granted deathtouch
//! until end of turn (one trigger per attacking Elf yields the same result
//! as the single "they gain" clause).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Tyvar the Bellicose");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let elf_filter = script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Each creature you control has 'Whenever a mana ability of this
    // creature resolves, put +1/+1 counters on it equal to the mana produced.
    // This ability triggers only once each turn.'" — a static grant of a
    // mana-ability-resolution trigger, with no primitive to express it.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks { filter: elf_filter },
            intervening_if: None,
            effect: grant_deathtouch_to_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_deathtouch_to_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Deathtouch,
        duration: Duration::EndOfTurn,
    }]
}
