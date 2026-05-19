//! Brazen Boarding — `{2}{U}{R}` sorcery. "Brazen Boarding deals 4 damage to target creature or
//! planeswalker. If excess damage is dealt this way, conjure a card with mana value equal to that
//! excess damage from Brazen Boarding's spellbook onto the battlefield. If that excess damage is
//! 4 or greater, instead conjure a card named Admiral Beckett Brass onto the battlefield."
//! GAP: conjure from spellbook (digital-only mechanic) — no Effect::Conjure variant; excess
//! damage tracking and conditional conjure also not expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brazen Boarding");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Brazen Boarding deals 4 damage to target creature or planeswalker. If excess damage is dealt this way, conjure a card with mana value equal to that excess damage from Brazen Boarding's spellbook onto the battlefield. If that excess damage is 4 or greater, instead conjure a card named Admiral Beckett Brass onto the battlefield.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine::CREATURE.into()),
                    ),
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DealDamage { source: entry.source, target: DamageTarget::Object(*id), amount: 4 },
        // GAP: conjure from spellbook based on excess damage — no Effect::Conjure variant;
        // excess damage tracking not expressible
    ]
}
