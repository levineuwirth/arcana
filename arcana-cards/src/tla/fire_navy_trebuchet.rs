//! Fire Navy Trebuchet — `{2}{B}` 0/4 Artifact Creature — Wall with
//! Defender and Reach.
//! "Whenever you attack, create a 2/1 colorless Construct artifact
//! creature token with flying named Ballistic Boulder that's tapped and
//! attacking. Sacrifice that token at the beginning of the next end
//! step."
//!
//! Defender and Reach are base keywords. There is no player-level
//! "whenever you attack" trigger, so this uses `CreatureAttacks`
//! filtered to creatures you control (fidelity gap: fires once per
//! attacking creature rather than once per combat). The token is minted
//! tapped and attacking via `CreateTokenTappedAttacking`. The
//! "sacrifice that token at the next end step" rider cannot target the
//! freshly-minted token (its id isn't known to schedule a delayed
//! sacrifice) — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire Navy Trebuchet");
    let wall = reg.interner_mut().intern("Wall");
    let _boulder = reg.interner_mut().intern("Ballistic Boulder");
    let _construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Reach],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: make_boulder,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_boulder(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let boulder = reg.interner().lookup("Ballistic Boulder").unwrap_or_default();
    let construct = reg.interner().lookup("Construct").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    // GAP: "Sacrifice that token at the beginning of the next end step" —
    // the freshly-minted token's id isn't available to schedule a delayed
    // sacrifice, and CreateTokenSacEot doesn't enter tapped-and-attacking.
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: TokenDefinition {
            name: boulder,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
