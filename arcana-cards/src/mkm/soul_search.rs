//! Soul Search — `{W}{B}` sorcery.
//! "Target opponent reveals their hand. You choose a nonland card from it.
//! Exile that card. If the card's mana value is 1 or less, create a 1/1
//! white and black Spirit creature token with flying."
//!
//! # GAP: hand-reveal + controller-chooses nonland card to exile not expressible.
//! GAP: "if the card's mana value is 1 or less" conditional (querying CMC of
//! the exiled card) not expressible via script API.
//! Best effort: discard from opponent, create Spirit token (always, since
//! conditional check is a GAP).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Search");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a nonland card from it. Exile that card. If the card's mana value is 1 or less, create a 1/1 white and black Spirit creature token with flying.".into(),
                target_requirements: vec![TargetRequirement::target_opponent()],
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
    // GAP: hand-reveal + controller-selects nonland card to exile not expressible.
    // GAP: CMC-conditional check on the exiled card not expressible; token always created.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let player = match target {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::Discard { player, count: 1, choice: DiscardChoice::OpponentChooses },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
