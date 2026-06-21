//! Mordor Trebuchet — `{2}{B}` 1/4 Artifact Creature — Wall.
//!
//! "Defender
//!  Whenever you attack with one or more Goblins and/or Orcs, create a
//!  2/1 colorless Construct artifact creature token with flying named
//!  Ballistic Boulder that's tapped and attacking. Sacrifice that token
//!  at end of combat."
//!
//! Decomposition: Defender keyword + a CreatureAttacks trigger (filtered
//! to Goblins/Orcs you control) that mints a tapped-and-attacking 2/1
//! flying Construct token. The trigger fires per matching attacker
//! (fidelity widening vs the once-per-attack-step batch). The "sacrifice
//! that token at end of combat" rider is GAP'd — the token's id is
//! engine-assigned and not available to schedule a delayed sacrifice.

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
    let name = reg.interner_mut().intern("Mordor Trebuchet");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    // Token / filter subtypes interned up front.
    let goblin = reg.interner_mut().intern("Goblin");
    let orc = reg.interner_mut().intern("Orc");
    let _construct = reg.interner_mut().intern("Construct");
    let _ballistic = reg.interner_mut().intern("Ballistic Boulder");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };
    let attacker_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![goblin, orc]);
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: attacker_filter,
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
    let token_name = reg.interner().lookup("Ballistic Boulder").unwrap_or_default();
    let construct = reg.interner().lookup("Construct").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    // GAP: "Sacrifice that token at end of combat." — the token's id is
    // engine-assigned and not available here to schedule a delayed sac.
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
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
