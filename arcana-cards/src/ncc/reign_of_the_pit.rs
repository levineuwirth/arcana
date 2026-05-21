//! Reign of the Pit — `{4}{B}{B}` sorcery. Each player sacrifices a
//! creature; create an X/X black Demon token with flying.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reign of the Pit");
    let _demon = reg.interner_mut().intern("Demon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player sacrifices a creature of their choice. Create an X/X black Demon creature token with flying, where X is the total power of the creatures sacrificed this way.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").expect("Demon interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    // GAP: 'total power of creatures sacrificed' — we can sequence
    // sacrifices, but cannot read their power retroactively to set X.
    // Sequence the per-player sacrifices and create a 0/0 token.
    let mut effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect();
    let token = TokenDefinition {
        name: demon,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
