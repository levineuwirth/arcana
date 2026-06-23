//! The Spot, Living Portal — `{3}{W}{B}` 4/4 Legendary Creature —
//! Human Scientist Villain.
//! When The Spot enters, exile up to one target nonland permanent and up to
//! one target nonland permanent card from a graveyard.
//! When The Spot dies, put him on the bottom of his owner's library. If you
//! do, return the exiled cards to their owners' hands.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Spot, Living Portal");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When The Spot enters, exile up to one target nonland permanent
            //  and up to one target nonland permanent card from a graveyard."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            // "When The Spot dies, put him on the bottom of his owner's library.
            //  If you do, return the exiled cards to their owners' hands."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_recur,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // First target: a nonland permanent — exile linked to The Spot, so the
    // engine auto-returns it to the battlefield when The Spot leaves.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::ExileUntilSourceLeaves {
            source: trig.source,
            target: *id,
        });
    }
    // Second target: a nonland permanent card in a graveyard.
    // (No exiled-with-source linkage for graveyard cards; the "return to hand
    //  on death" of THIS exiled card is a fidelity gap — it is plainly exiled.)
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.get(1) {
        effects.push(Effect::ExileFromGraveyard { target: *id });
    }
    effects
}

fn dies_recur(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Put The Spot on the bottom of its owner's library. The exiled nonland
    // permanent linked via ExileUntilSourceLeaves auto-returns when The Spot
    // left the battlefield (to the battlefield rather than hand — a fidelity
    // gap), so only the self-bounce is emitted here.
    vec![Effect::PutOnBottomOfLibrary { target: trig.source }]
}
