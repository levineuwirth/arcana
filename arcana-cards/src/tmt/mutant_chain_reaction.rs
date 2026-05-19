//! Mutant Chain Reaction — `{2}{G}` sorcery. "Destroy up to one target artifact,
//! enchantment, or creature with flying. Create a Mutagen token."
//!
//! # GAP: Mutagen token has an activated ability ("{1},{T},Sacrifice: +1/+1 counter")
//! which is not expressible in TokenDefinition. Token created without the ability.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mutant Chain Reaction");
    let _mutagen = reg.interner_mut().intern("Mutagen");
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
                text: "Destroy up to one target artifact, enchantment, or creature with flying. Create a Mutagen token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT | TypeLine::CREATURE),
                        ),
                    ),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg.interner().lookup("Mutagen").expect("Mutagen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen);
    let token = TokenDefinition {
        name: mutagen,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: activated ability "{1},{T}, Sacrifice: put +1/+1 counter on target creature" not expressible
        abilities: vec![],
    };
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
