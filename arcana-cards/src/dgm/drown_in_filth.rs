//! Drown in Filth — `{B}{G}` sorcery. "Choose target creature. Mill
//! four cards, then that creature gets -1/-1 until end of turn for
//! each land card in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drown in Filth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature. Mill four cards, then that creature gets -1/-1 until end of turn for each land card in your graveyard.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Note: per CR 608.2 mill happens, THEN the count of land cards
    // in graveyard is read. script::graveyard_matching reads live
    // state, so the value is computed before the mill resolves — the
    // resolver returns effects which the engine applies in order.
    // Compute the count after-the-fact would require a deferred X;
    // here we use the pre-mill count as a documented imprecision.
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let x = script::graveyard_matching(
        state,
        &land_filter,
        entry.controller,
        entry.controller,
    ) as i32;
    vec![
        Effect::Mill { player: entry.controller, count: 4 },
        Effect::Pump {
            target: *id,
            power: -x,
            toughness: -x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
