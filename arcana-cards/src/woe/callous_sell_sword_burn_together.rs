//! Callous Sell-Sword // Burn Together — `{1}{B}` / `{R}` Adventure
//!
//! Creature: `{1}{B}` Creature — Human Soldier (2/2)
//!   Enters with a +1/+1 counter for each creature that died under your
//!   control this turn. (GAP: no "died this turn for you" overall counter —
//!   using creatures_of_subtype_died_this_turn with empty-subtype workaround
//!   is not possible; emitting 0 counters as approximation.)
//!
//! Adventure: `{R}` Sorcery — Burn Together
//!   Target creature you control deals damage equal to its power to any other
//!   target. Then sacrifice it.
//!   (GAP: "creature you control deals damage to another target" — Fight
//!   requires two creatures; using DealDamage from source with power lookup.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Callous Sell-Sword");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Burn Together");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target creature you control deals damage equal to its power to any other target. Then sacrifice it.".into(),
        target_requirements: vec![
            TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::You)),
                count: TargetCount::Exactly(1), controller: None,
            },
            TargetRequirement::any_target(),
        ],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    if entry.targets.targets.len() < 2 { return Vec::new(); }
    let TargetChoice::Object(source_id) = &entry.targets.targets[0] else { return Vec::new(); };
    let power = script::power_of(state, *source_id).max(0) as u32;
    let dt = match &entry.targets.targets[1] {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    let filter = ObjectFilter::creature().controlled_by(arcana_core::targets::ControllerConstraint::You);
    vec![
        Effect::DealDamage { target: dt, amount: power, source: entry.source },
        Effect::Sacrifice { player: entry.controller, filter, count: 1 },
    ]
}
