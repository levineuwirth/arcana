//! Crash Landing — `{2}{G}` instant. "Target creature with flying
//! loses flying until end of turn. Crash Landing deals damage to that
//! creature equal to the number of Forests you control."
//! "Loses flying" installs a targeted `ContinuousEffect::remove_keyword`
//! (Layer 6) before the damage.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crash Landing");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                // GAP: the "with flying" target restriction isn't
                // keyword-filtered here; target is any creature.
                text: "Target creature with flying loses flying until end of turn. Crash Landing deals damage to that creature equal to the number of Forests you control.".into(),
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
    let forests = script::count_matching(
        state,
        &script::subtype_filter(reg, "Forest")
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![
        // "Target creature with flying loses flying until end of turn"
        // — targeted keyword removal (Layer 6), installed before the
        // damage per the oracle order.
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::remove_keyword(
                entry.source,
                *id,
                KeywordAbility::Flying,
                Duration::EndOfTurn,
            ),
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: forests,
        },
    ]
}
