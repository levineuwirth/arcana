//! Waltz of Rage — `{3}{R}{R}` sorcery, "Target creature you control
//! deals damage equal to its power to each other creature. Until end of
//! turn, whenever a creature you control dies, exile the top card of
//! your library. You may play it until the end of your next turn." The
//! delayed dies-triggered impulse is not expressible; the spread damage
//! is.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waltz of Rage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its \
                   power to each other creature. Until end of turn, \
                   whenever a creature you control dies, exile the top card \
                   of your library. You may play it until the end of your \
                   next turn."
                .into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(src)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let pw = script::power_of(state, *src).max(0) as u32;
    let ids: Vec<_> =
        script::ids_matching(state, &ObjectFilter::creature(), entry.controller)
            .into_iter()
            .filter(|id| id != src)
            .collect();
    // GAP: the "until end of turn, whenever a creature you control dies,
    // impulse-draw" delayed trigger is not expressible.
    ids.into_iter()
        .map(|id| Effect::DealDamage {
            source: *src,
            target: DamageTarget::Object(id),
            amount: pw,
        })
        .collect()
}
