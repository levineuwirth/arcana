//! Hour of Promise — `{4}{G}` sorcery. "Search your library for up to two
//! land cards, put them onto the battlefield tapped, then shuffle. Then if
//! you control three or more Deserts, create two 2/2 black Zombie creature
//! tokens."

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
    let name = reg.interner_mut().intern("Hour of Promise");
    let _zombie = reg.interner_mut().intern("Zombie");
    let _desert = reg.interner_mut().intern("Desert");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to two land cards, put them onto the battlefield tapped, then shuffle. Then if you control three or more Deserts, create two 2/2 black Zombie creature tokens.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let mut effects = vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land.clone(),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land,
            tapped: true,
        },
    ];
    let deserts = script::count_matching(
        state,
        &script::subtype_filter(reg, "Desert").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if deserts >= 3 {
        let zombie = reg.interner().lookup("Zombie").expect("Zombie interned");
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(zombie);
        let token = TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: token.clone(),
        });
        effects.push(Effect::CreateToken { controller: entry.controller, token });
    }
    effects
}
