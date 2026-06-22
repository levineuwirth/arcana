//! Jacob Frye — `{2}{B}` 3/2 Legendary Human Assassin.
//! Partner with Evie Frye (ETB: target player may put Evie into their hand
//! from their library, then shuffle).
//! Whenever one or more Assassins you control deal combat damage to a player,
//! exile up to one target Assassin card or card with freerunning from your
//! graveyard. If you do, copy it. You may cast the copy.
//!
//! Partner / Partner with are not usable KeywordAbility variants — GAP'd.
//! The "Partner with Evie Frye" ETB is modeled as a tutor-by-name to the
//! chosen player's hand (the optional "may" is a fidelity gap; a failed
//! search is a no-op). The combat-damage trigger ("exile a graveyard card,
//! copy it, you may cast the copy") is GAP'd — there is no primitive that
//! exiles a graveyard card and casts a copy of it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jacob Frye");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    // GAP: keywords "Partner with" / "Partner" have no usable KeywordAbility variants.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: partner_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: graveyard_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn partner_tutor(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let nm = reg.interner().lookup("Evie Frye");
    // GAP fidelity: the "may" is a resolution choice; a failed search no-ops.
    vec![Effect::TutorToHand {
        player: *p,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: false,
    }]
}

fn graveyard_copy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile up to one target Assassin/freerunning card from your
    // graveyard, copy it, you may cast the copy" — no primitive exiles a
    // graveyard card and casts a copy of it.
    Vec::new()
}
