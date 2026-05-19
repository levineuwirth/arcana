//! Salt Road Skirmish — `{3}{B}` sorcery, "Destroy target creature. Create
//! two 1/1 red Warrior creature tokens. They gain haste until end of turn.
//! Sacrifice them at the beginning of the next end step."
//! GAP: delayed sacrifice trigger (sacrifice at beginning of next end step)
//! is not expressible with the current Effect catalog.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Salt Road Skirmish");
    let _warrior = reg.interner_mut().intern("Warrior");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Create two 1/1 red Warrior creature tokens. They gain haste until end of turn. Sacrifice them at the beginning of the next end step.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    // GAP: sacrifice tokens at the beginning of the next end step
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
