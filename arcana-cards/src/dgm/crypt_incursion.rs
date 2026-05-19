//! Crypt Incursion — `{2}{B}` instant, "Exile all creature cards from target player's
//! graveyard. You gain 3 life for each card exiled this way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crypt Incursion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creature cards from target player's graveyard. You gain 3 life for each card exiled this way.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let TargetChoice::Player(target_player) = target else { return Vec::new(); };
    let count = script::graveyard_matching(state, &ObjectFilter::creature(), *target_player, entry.controller);
    let life_gain = count * 3;
    // GAP: exiling individual cards from graveyard by filter (all at once) not directly expressible;
    // using Mill-style approach is not accurate. Best effort: gain life only.
    // GAP: ExileFromGraveyard takes a single ObjectId, not a filter over all cards in a zone.
    let mut effects = Vec::new();
    if life_gain > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: life_gain });
    }
    effects
}
