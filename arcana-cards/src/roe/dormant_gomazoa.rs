//! Dormant Gomazoa — `{1}{U}{U}` 5/5 Creature — Jellyfish.
//! Flying.
//! This creature enters tapped. (entry replacement — GAP)
//! This creature doesn't untap during your untap step.
//! Whenever you become the target of a spell, you may untap this creature.
//!
//! The "doesn't untap during your untap step" static is now wired via the
//! continuous-effect idiom: a `SelfEntersBattlefield` trigger installs a
//! `ContinuousEffect::dont_untap` anchored to this creature, lasting while it
//! stays on the battlefield. The remaining clauses stay GAP'd:
//! - "enters tapped" is an entry replacement (no Effect/trigger for it).
//! - "Whenever YOU (the player) become the target of a spell" — there is no
//!   player-becomes-target TriggerCondition; SelfBecomesTarget watches the
//!   creature, not its controller.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dormant Gomazoa");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);

    // GAP: "This creature enters tapped." — entry replacement, no API surface.
    // GAP: trigger — "Whenever you become the target of a spell" has no
    //       player-becomes-target TriggerCondition variant.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_dont_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "This creature doesn't untap during your untap step." Install an
/// unconditional don't-untap restriction on the creature itself,
/// auto-expiring when it leaves the battlefield.
fn install_dont_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::dont_untap(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
