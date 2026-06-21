//! Lazav, Wearer of Faces — `{U}{B}` 2/3 Legendary Shapeshifter
//! Detective.
//! "Whenever Lazav attacks, exile target card from a graveyard, then
//! investigate."
//! "Whenever you sacrifice a Clue, you may have Lazav become a copy of
//! a creature card exiled with it until end of turn."

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lazav, Wearer of Faces");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let detective = reg.interner_mut().intern("Detective");
    let clue = reg.interner_mut().intern("Clue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // Scryfall keyword "Investigate" is the action verb in the attack trigger,
    // not a keyword-line ability — handled in the trigger body via a Clue token.
    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever Lazav attacks, exile target card from a graveyard, then
            // investigate."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_exile_then_investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // "Whenever you sacrifice a Clue, …"
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::ARTIFACT.into())
                        .with_subtype_sym(clue)
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_sac_clue,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_exile_then_investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        out.push(Effect::ExileFromGraveyard { target: *id });
    }
    // "then investigate" — create a Clue token.
    out.push(Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    });
    out
}

fn on_sac_clue(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Lazav becomes a copy of a creature card exiled with that Clue" —
    // there is no linkage from a sacrificed Clue to the card exiled alongside
    // it, and no "become a copy of a card in exile" effect; omitted.
    Vec::new()
}
