//! Din of the Fireherd — `{5}{B/R}{B/R}{B/R}` sorcery. "Create a 5/5
//! black and red Elemental creature token. Target opponent sacrifices
//! a creature of their choice for each black creature you control,
//! then sacrifices a land of their choice for each red creature you
//! control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 5/5 black and red Elemental creature token. Target opponent sacrifices a creature of their choice for each black creature you control, then sacrifices a land of their choice for each red creature you control.".into(),
            target_requirements: vec![TargetRequirement::target_opponent()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }];
    let black_count = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::black()),
        entry.controller,
    );
    let red_count = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::red()),
        entry.controller,
    );
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.first() {
        if black_count > 0 {
            effects.push(Effect::Sacrifice {
                player: *p,
                filter: ObjectFilter::creature(),
                count: black_count,
            });
        }
        if red_count > 0 {
            effects.push(Effect::Sacrifice {
                player: *p,
                filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                count: red_count,
            });
        }
    }
    effects
}
