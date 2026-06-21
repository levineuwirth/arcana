//! Carnage, Crimson Chaos — `{2}{B}{R}` 4/3 Legendary Symbiote Villain.
//! Trample.
//! When Carnage enters, return target creature card with mana value 3 or
//! less from your graveyard to the battlefield. It gains "This creature
//! attacks each combat if able" and "When this creature deals combat damage
//! to a player, sacrifice it."
//! Mayhem {B}{R}.
//!
//! Trample is wired. The ETB reanimation is wired (target creature card,
//! mv <= 3, in your graveyard -> battlefield) and the returned creature is
//! granted the "deals combat damage to a player -> sacrifice it" triggered
//! ability via `GrantTriggeredAbility`. Two parts are GAP'd: the gained
//! "attacks each combat if able" static (no faithful must-attack primitive —
//! Goad adds a can't-attack-goader rider) and Mayhem (not in the usable
//! keyword surface; its graveyard-cast mechanic is not expressible).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carnage, Crimson Chaos");
    let symbiote = reg.interner_mut().intern("Symbiote");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(symbiote);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Mayhem {B}{R} — not in the usable keyword surface.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You)
                            .with_max_cmc(3),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_reanimate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // It gains "When this creature deals combat damage to a player, sacrifice it."
    let granted = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::default(),
            target_filter: TargetFilter::Player,
            combat_only: true,
        },
        intervening_if: None,
        effect: granted_sacrifice_self,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    // GAP: gained "This creature attacks each combat if able." — no faithful
    // must-attack primitive (Goad carries a can't-attack-goader rider).
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(granted),
            duration: arcana_core::layers::Duration::WhileSourceOnBattlefield,
        },
    ]
}

fn granted_sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Phase-1 self-sacrifice routes through DestroyPermanent.
    vec![Effect::DestroyPermanent { target: trig.source }]
}
