//! Fists of Flame — `{1}{R}` instant. "Draw a card. Until end of turn,
//! target creature gains trample and gets +1/+0 for each card you've
//! drawn this turn."
//!
//! The draw and the trample grant are expressible; the +1/+0 pump is
//! dynamic on "cards you've drawn this turn", a quantity no `script::`
//! helper exposes, so that part is GAP-ed rather than emitted with a
//! wrong literal.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fists of Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card. Until end of turn, target creature gains \
                       trample and gets +1/+0 for each card you've drawn this \
                       turn."
                    .into(),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return vec![Effect::DrawCards { player: entry.controller, count: 1 }];
    };
    // GAP: "+1/+0 for each card you've drawn this turn" is dynamic on the
    // count of cards drawn this turn, which no script:: helper exposes; the
    // dynamic pump is omitted. The draw and the trample grant are emitted.
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}
