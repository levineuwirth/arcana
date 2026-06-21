//! Roadkill Rodney — `{2}` 2/1 colorless Artifact Creature — Robot.
//! Squad {3}; Deathtouch; "Whenever this creature deals combat damage to
//! a player, create a Mutagen token."
//!
//! Deathtouch is a base keyword. Squad {3} is an additional cast cost
//! ("pay {3} any number of times; on ETB create that many copies") with
//! no expressible cost/ETB-copy machinery — GAP'd. The combat-damage
//! trigger mints a Mutagen artifact token (its own activated ability is
//! GAP'd — modeled as a bare colorless artifact token).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roadkill Rodney");
    let robot = reg.interner_mut().intern("Robot");
    // Pre-intern the Mutagen token name for the resolver.
    let _mutagen = reg.interner_mut().intern("Mutagen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    // GAP (Squad {3}): additional-cost "pay {3} any number of times" with an
    // ETB "create that many copies of this" rider — no expressible cast-cost
    // multiplier or copy-on-enter machinery.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_mutagen,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_mutagen(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg.interner().lookup("Mutagen").unwrap_or_default();
    // GAP: the Mutagen token's own activated ability is unmodeled — minted as
    // a bare colorless artifact token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: mutagen,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
