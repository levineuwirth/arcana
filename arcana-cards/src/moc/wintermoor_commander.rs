//! Wintermoor Commander — `{W}{B}` 2/* Human Knight with Deathtouch.
//! "Wintermoor Commander's toughness is equal to the number of Knights you
//!  control." (a characteristic-defining toughness — `*`)
//! "Whenever this creature attacks, another target Knight you control gains
//!  indestructible until end of turn."
//!
//! Toughness is `PtValue::Star`. GAP: the CDA is an ASYMMETRIC-SUBTYPE one
//! (power fixed 2, toughness = Knights you control). self_pt_from_match would
//! SET both P/T to the Knight count (clobbering the fixed 2 power), and
//! self_pt_cda has no registry to resolve the "Knight" subtype — so neither
//! constructor applies. The attack trigger grants indestructible to another
//! target Knight you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wintermoor Commander");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        // GAP: asymmetric-subtype CDA "toughness equal to the number of
        // Knights you control" (power stays fixed 2) — not expressible by
        // self_pt_from_match (sets BOTH P/T) or self_pt_cda (no subtype
        // registry); toughness is `*`.
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // "another target Knight you control".
    let knight_filter =
        script::subtype_filter(reg, "Knight").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: grant_indestructible,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(knight_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn grant_indestructible(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another target Knight you control" — best-effort: the chosen target.
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}
