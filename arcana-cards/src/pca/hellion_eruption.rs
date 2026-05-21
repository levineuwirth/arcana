//! Hellion Eruption — `{5}{R}` sorcery. "Sacrifice all creatures you
//! control, then create that many 4/4 red Hellion creature tokens."
//! Sacrifice-all isn't a primitive — we count creatures you control
//! up front, then issue that many sacrifices and that many tokens.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hellion Eruption");
    let _hellion = reg.interner_mut().intern("Hellion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Sacrifice all creatures you control, then create that many 4/4 red Hellion creature tokens.".into(),
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
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let hellion = reg.interner().lookup("Hellion").expect("Hellion interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    let token = TokenDefinition {
        name: hellion,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::Sacrifice {
        player: entry.controller,
        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        count: n,
    }];
    for _ in 0..n {
        effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    }
    effects
}
