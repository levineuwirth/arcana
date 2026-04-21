//! Snapcaster Mage — `{1}{U}` 2/1 Human Wizard. "When Snapcaster
//! Mage enters the battlefield, target instant or sorcery card in a
//! graveyard gains flashback until end of turn. The flashback cost
//! is equal to its mana cost."
//!
//! # Why this card matters
//!
//! Snapcaster is the first card in the seed set that:
//! 1. Targets a card in a **non-battlefield zone** (graveyard) —
//!    exercises TargetFilter::Card / graveyard enumeration.
//! 2. Grants a **keyword to a graveyard card** — exercises the
//!    layer system applying continuous effects to non-battlefield
//!    objects (compute_characteristics doesn't filter by zone, so
//!    the grant is visible).
//! 3. Has a **targeted triggered ability** — the trigger fires from
//!    the battlefield but picks its target from a graveyard.
//!
//! # Scope note (mid-resolution target choice)
//!
//! CR 603.3d says targets for a triggered ability are chosen as the
//! trigger is being put on the stack. Phase 2-A's trigger pipeline
//! pushes triggers with empty targets (`StackEntry::new_triggered_ability`
//! at engine.rs passes `TargetSelection::new()`), and Snapcaster
//! resolves its target choice *at resolution time* instead. For
//! Snapcaster this is observably equivalent — nothing between trigger
//! and resolution can change which instants/sorceries are in
//! graveyards. For a future targeted trigger that can be disrupted
//! mid-flight, we'll need to move target choice to trigger-put-on-
//! stack (phase-2-B).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Snapcaster Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_grant_flashback,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: push a `PickCards` prompt listing instants/sorceries
/// in any graveyard, and hand off to the
/// `GrantFlashbackEqualToOwnManaCost` follow-up.
fn etb_grant_flashback(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantFlashbackToInstantOrSorceryInGraveyard {
        source: trig.source,
        controller: trig.controller,
        duration: Duration::EndOfTurn,
    }]
}
