//! Cheeky House-Mouse // Squeak By — `{W}` // `{W}` white Adventure creature.
//! Creature: 2/1 Mouse. No oracle abilities printed.
//! Adventure (Squeak By — Sorcery): Target creature you control gets +1/+1 until end of turn.
//!   It can't be blocked by creatures with power 3 or greater this turn.
//! GAP: "can't be blocked by creatures with power 3 or greater" — block-restriction by power not in catalog.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cheeky House-Mouse");
    let adv_name = reg.interner_mut().intern("Squeak By");
    let mouse_sub = reg.interner_mut().intern("Mouse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target creature you control gets +1/+1 until end of turn. It can't be blocked by creatures with power 3 or greater this turn.".into(),
        target_requirements: vec![TargetRequirement::target_creature()],
        modal: None,
        effect: squeak_by_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure),
    )
}

fn squeak_by_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "can't be blocked by creatures with power 3 or greater" — block-restriction by power not in catalog
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
