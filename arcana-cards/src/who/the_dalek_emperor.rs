//! The Dalek Emperor — `{5}{B}{R}` 6/6 Legendary Artifact Creature — Dalek.
//!
//! Oracle:
//! * Affinity for Daleks — a cost-reduction static. `Affinity` is not a usable
//!   `KeywordAbility` variant and the cost-reduction static is not expressible,
//!   so it is GAP'd (no keyword emitted).
//! * Other Daleks you control have haste — a STATIC continuous anthem over your
//!   other Daleks; not a triggered/activated ability and not expressible as a
//!   board-wide keyword anthem here. GAP'd.
//! * At the beginning of combat on your turn, each opponent faces a villainous
//!   choice — sacrifice a creature of their choice, OR you create a 3/3 black
//!   Dalek artifact creature token with menace. The "villainous choice" (the
//!   opponent picks which half happens) is not expressible; we faithfully model
//!   the upside half — creating one token per opponent — and GAP the
//!   opponent-chosen sacrifice branch.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Dalek Emperor");
    let dalek = reg.interner_mut().intern("Dalek");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dalek);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Affinity for Daleks — cost reduction; not a usable keyword and
        // not expressible.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Other Daleks you control have haste" (board-wide anthem).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_villainous_choice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Combat trigger: each opponent faces a villainous choice. We model the upside
/// (create a 3/3 black Dalek artifact creature token with menace) once per
/// opponent. GAP: the opponent-chosen "sacrifice a creature instead" branch is
/// a player-driven choice not expressible here.
fn combat_villainous_choice(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dalek = reg.interner().lookup("Dalek").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dalek);
    let opponents = script::opponents(state, trig.controller);
    opponents
        .into_iter()
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: dalek,
                colors: ColorSet::black(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![KeywordAbility::Menace],
                abilities: vec![],
            },
        })
        .collect()
}
