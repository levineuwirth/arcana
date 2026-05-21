//! Psychic Whorl — `{2}{B}` sorcery. "Target opponent discards two
//! cards. Then if you control a Rat, surveil 2." The conditional
//! 'if you control a Rat' surveil is approximated: we emit the
//! discards and an unconditional surveil (best-effort partial).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychic Whorl");
    let _rat = reg.interner_mut().intern("Rat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent discards two cards. Then if you control a Rat, surveil 2.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::ControllerChooses,
    }];
    let rats = script::count_matching(
        state,
        &script::subtype_filter(reg, "Rat").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if rats > 0 {
        effects.push(Effect::Surveil { player: entry.controller, count: 2 });
    }
    effects
}
