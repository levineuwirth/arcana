//! Fragment Reality — `{W}` instant. "Exile target nontoken artifact, creature, or enchantment an
//! opponent controls. Its controller puts a random creature card with lesser mana value from their
//! library onto the battlefield tapped."
//! GAP: its controller searches library for a random creature with lesser mana value than exiled
//! permanent and puts it onto battlefield tapped — no Effect variant for random tutor-to-battlefield
//! gated on mana value comparison, no "tapped" flag on TutorToBattlefield; target restriction to
//! opponent-controlled nontoken also not expressible via ObjectFilter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fragment Reality");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target nontoken artifact, creature, or enchantment an opponent controls. Its controller puts a random creature card with lesser mana value from their library onto the battlefield tapped.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine::ARTIFACT.into())
                            .with_types_any(TypeLine::CREATURE.into())
                            .with_types_any(TypeLine::ENCHANTMENT.into()),
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
        Effect::ExilePermanent { target: *id },
        // GAP: its controller puts a random creature card with lesser mana value from
        // their library onto the battlefield tapped — no random tutor-to-battlefield
        // gated on mana value comparison; no tapped entry flag on TutorToBattlefield
    ]
}
