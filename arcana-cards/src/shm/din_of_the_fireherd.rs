//! Din of the Fireherd — `{5}{B/R}{B/R}{B/R}` sorcery. "Create a 5/5 black and red Elemental
//! creature token. Target opponent sacrifices a creature of their choice for each black creature
//! you control, then sacrifices a land of their choice for each red creature you control."
//! GAP: opponent sacrifices N creatures (count = your black creatures) then N lands (count = your
//! red creatures) — no Effect::Sacrifice variant exists; the dynamic count based on your board
//! state is also not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Din of the Fireherd");
    let _elemental = reg.interner_mut().intern("Elemental");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B/R}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 5/5 black and red Elemental creature token. Target opponent sacrifices a creature of their choice for each black creature you control, then sacrifices a land of their choice for each red creature you control.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token },
        // GAP: opponent sacrifices N creatures (N = your black creatures on battlefield) then
        // N lands (N = your red creatures) — no Effect::Sacrifice variant, dynamic count
        // not expressible
    ]
}
