//! Spider Food — `{2}{G}` sorcery, "Destroy up to one target artifact,
//! enchantment, or creature with flying. Create a Food token."
//!
//! GAP: "artifact, enchantment, or creature with flying" as a unified target
//! filter (OR of types + has-flying predicate) not expressible in ObjectFilter;
//! GAP: Food token (named artifact token with activated life-gain ability) not
//! in TokenDefinition shape.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spider Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy up to one target artifact, enchantment, or creature with flying. Create a Food token. (It's an artifact with \"{2}, {T}, Sacrifice this token: You gain 3 life.\")".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::UpTo(1),
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
    use arcana_core::targets::TargetChoice;
    // GAP: artifact/enchantment/flying-creature OR-filter not expressible in ObjectFilter
    // GAP: Food token (named artifact with activated ability) not in TokenDefinition
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}
