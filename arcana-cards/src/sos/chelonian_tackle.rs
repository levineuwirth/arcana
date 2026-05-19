//! Chelonian Tackle — `{2}{G}` sorcery. "Target creature you control gets
//! +0/+10 until end of turn. Then it fights up to one target creature an
//! opponent controls."
//!
//! GAP: DealDamageEqualToPower (fight is one-sided in resolve; true fight
//! requires mutual damage which is approximated with Effect::Fight).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chelonian Tackle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +0/+10 until end of turn. Then it fights up to one target creature an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Permanent(
                            arcana_core::targets::ObjectFilter::creature(),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut targets = entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(*id) } else { None }
    });
    let Some(a) = targets.next() else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: a,
        power: 0,
        toughness: 10,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(b) = targets.next() {
        effects.push(Effect::Fight { a, b });
    }
    effects
}
