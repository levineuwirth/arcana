//! Invoke Despair — `{1}{B}{B}{B}{B}` sorcery. "Target opponent sacrifices a
//! creature of their choice. If they can't, they lose 2 life and you draw a
//! card. Then repeat this process for an enchantment and a planeswalker."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invoke Despair");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent sacrifices a creature of their choice. If they can't, they lose 2 life and you draw a card. Then repeat this process for an enchantment and a planeswalker.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let p = match target {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    // GAP: conditional "if they can't sacrifice" — engine cannot branch on whether the sacrifice succeeded.
    // Emit only the sacrifices for each of the three permanent types.
    vec![
        Effect::Sacrifice { player: p, filter: ObjectFilter::creature(), count: 1 },
        Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into()), count: 1 },
        Effect::Sacrifice { player: p, filter: ObjectFilter::permanent().with_types(TypeLine::PLANESWALKER.into()), count: 1 },
    ]
}
