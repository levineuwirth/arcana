//! Excogitator Sphinx — `{2}{U}{U}` 4/4 Sphinx Detective.
//!
//! Flying
//! Whenever one or more creatures you control deal combat damage to a
//! player, investigate. (Modeled as creating a Clue token via
//! `Effect::CreateCommodityToken`.)
//! `{1}, Sacrifice a Clue: Seek an instant or sorcery card.`
//!
//! GAP: keyword line `Investigate` / `Seek` are not in the usable
//! `KeywordAbility` surface — `keywords` is just `Flying`. Investigate
//! is wired as a Clue commodity token. Seek (CR 701.55) has no engine
//! `Effect`, so the activated ability is GAP'd (returns `Vec::new()`).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Excogitator Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let detective = reg.interner_mut().intern("Detective");
    let clue = reg.interner_mut().intern("Clue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice a Clue: Seek an instant or sorcery card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter::new().with_subtype_sym(clue)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: seek_gap,
            }),
    )
}

/// "Investigate" — create a Clue token.
fn investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}

/// GAP: Seek (CR 701.55, "reveal cards from your library at random until
/// you reveal a matching card, put it into your hand") has no engine
/// `Effect` variant. No fixed-count library search matches its random
/// reveal semantics faithfully — emit nothing rather than invent.
fn seek_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
