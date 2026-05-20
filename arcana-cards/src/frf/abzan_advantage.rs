//! Abzan Advantage — `{1}{W}` instant. "Target player sacrifices an
//! enchantment of their choice. Bolster 1."
//!
//! Bolster (put a +1/+1 counter on the creature with the least
//! toughness among creatures you control) has no catalog Effect; we
//! express the forced enchantment sacrifice and gap the bolster.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abzan Advantage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player sacrifices an enchantment of their choice. Bolster 1.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: Bolster 1 (least-toughness +1/+1 counter) has no catalog
    // Effect.
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        count: 1,
    }]
}
