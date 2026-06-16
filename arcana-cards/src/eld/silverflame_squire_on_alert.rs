//! Silverflame Squire // On Alert — `{1}{W}` Human Soldier 2/1 creature /
//! `{2}{W}` Instant — Adventure
//!
//! On Alert: Target creature gets +2/+2 until end of turn. Untap it.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silverflame Squire");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("On Alert");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid adv cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target creature gets +2/+2 until end of turn. Untap it.".into(),
        target_requirements: vec![TargetRequirement::target_creature()],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn adv_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Untap { target: *id },
    ]
}
