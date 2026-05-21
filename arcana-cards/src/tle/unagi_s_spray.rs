//! Unagi's Spray — `{U}` instant. "Target creature gets -4/-0 until
//! end of turn. If you control a Fish, Octopus, Otter, Seal, Serpent,
//! or Whale, draw a card."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unagi's Spray");
    for st in ["Fish", "Octopus", "Otter", "Seal", "Serpent", "Whale"] {
        reg.interner_mut().intern(st);
    }
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -4/-0 until end of turn. If you control a Fish, Octopus, Otter, Seal, Serpent, or Whale, draw a card.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: -4,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    let controls_aquatic = ["Fish", "Octopus", "Otter", "Seal", "Serpent", "Whale"]
        .iter()
        .any(|st| {
            script::count_matching(
                state,
                &script::subtype_filter(reg, st)
                    .controlled_by(ControllerConstraint::You),
                entry.controller,
            ) > 0
        });
    if controls_aquatic {
        effects.push(Effect::DrawCards { player: entry.controller, count: 1 });
    }
    effects
}
