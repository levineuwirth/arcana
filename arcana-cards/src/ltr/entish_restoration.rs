//! Entish Restoration — `{2}{G}` instant. "Sacrifice a land. Search your
//! library for up to two basic land cards, put them onto the battlefield
//! tapped, then shuffle. If you control a creature with power 4 or
//! greater, instead search for up to three basic land cards…"

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Entish Restoration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Sacrifice a land. Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle. If you control a creature with power 4 or greater, instead search your library for up to three basic land cards, put them onto the battlefield tapped, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let has_big = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    ) > 0;
    let copies = if has_big { 3 } else { 2 };
    let land_filter = || ObjectFilter::new().with_types(TypeLine::LAND.into());
    let mut out: Vec<Effect> = vec![Effect::Sacrifice {
        player: entry.controller,
        filter: land_filter(),
        count: 1,
    }];
    // GAP: "basic" supertype restriction not exposed — best-effort with
    // a plain land filter. The engine has no "enters tapped" rider on
    // TutorToBattlefield either, so that's also missing.
    for _ in 0..copies {
        out.push(Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land_filter(),
            tapped: true,
        });
    }
    out
}
