//! Flash Conscription — `{5}{R}` instant. "Untap target creature and
//! gain control of it until end of turn. That creature gains haste
//! until end of turn. If {W} was spent to cast this spell, the creature
//! gains 'Whenever this creature deals combat damage, you gain that
//! much life' until end of turn."
//!
//! GAP: GainControl (no Effect::GainControl variant in catalog).
//! GAP: HybridManaCondition (checking whether {W} was spent).
//! GAP: GrantTriggeredAbility (granting a triggered ability until EOT).
//! Untap and Haste grant are expressed; the rest is noted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flash Conscription");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Untap target creature and gain control of it until end of turn. That creature gains haste until end of turn. If {W} was spent to cast this spell, the creature gains \"Whenever this creature deals combat damage, you gain that much life\" until end of turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: GainControl (no Effect::GainControl variant)
    // GAP: HybridManaCondition (checking whether {W} was spent)
    // GAP: GrantTriggeredAbility (lifelink-on-combat-damage trigger until EOT)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
