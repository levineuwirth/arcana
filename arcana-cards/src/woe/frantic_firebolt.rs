//! Frantic Firebolt — `{2}{R}` instant.
//! "Frantic Firebolt deals X damage to target creature, where X is 2 plus the number of cards
//! in your graveyard that are instant cards, sorcery cards, and/or have an Adventure."
//! GAP: counting graveyard cards filtered to instant/sorcery/Adventure types is not expressible with graveyard_matching helper.
//! Best effort: deal 2 damage (base only, graveyard count omitted).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frantic Firebolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Frantic Firebolt deals X damage to target creature, where X is 2 plus the number of cards in your graveyard that are instant cards, sorcery cards, and/or have an Adventure.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: counting graveyard instant/sorcery/Adventure cards is not expressible
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 2,
    }]
}
