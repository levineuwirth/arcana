//! Dire Tactics — `{W}{B}` instant. "Exile target creature. If you don't
//! control a Human, you lose life equal to that creature's toughness." The
//! conditional life loss reads the exiled creature's toughness; toughness
//! is read before exile via the script helper.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dire Tactics");
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature. If you don't control a Human, you lose life equal to that creature's toughness.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let toughness = script::toughness_of(state, *id).max(0) as u32;
    let humans = script::count_matching(
        state,
        &script::subtype_filter(reg, "Human").controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let mut effects = vec![Effect::ExilePermanent { target: *id }];
    if humans == 0 {
        effects.push(Effect::LoseLife { player: entry.controller, amount: toughness });
    }
    effects
}
