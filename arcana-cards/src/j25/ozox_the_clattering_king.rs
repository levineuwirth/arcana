//! Ozox, the Clattering King — `{2}{B}` 3/2 Legendary Skeleton Noble.
//! "Ozox can't block."
//! "When Ozox dies, create Jumblebones, a legendary 2/1 black Skeleton
//! creature token with 'Jumblebones can't block' and 'When Jumblebones
//! leaves the battlefield, return target card named Ozox, the Clattering
//! King from your graveyard to your hand.'"
//!
//! GAP: the static "Ozox can't block" (and the token's matching "can't
//! block" clause) is a pure continuous self-restriction with no
//! triggered/activated hook. The dies trigger and the token's
//! leaves-the-battlefield return trigger are wired.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ozox, the Clattering King");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let noble = reg.interner_mut().intern("Noble");
    let jumblebones = reg.interner_mut().intern("Jumblebones");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(noble);
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
    // `jumblebones` is interned above; the token resolver recovers it
    // via reg.interner().lookup("Jumblebones").
    let _ = jumblebones;
    // GAP: static "Ozox can't block." — no expressible hook.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_create_jumblebones,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_create_jumblebones(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let jumblebones = reg.interner().lookup("Jumblebones")
        .expect("Jumblebones interned during register()");
    let skeleton = reg.interner().lookup("Skeleton")
        .expect("Skeleton interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    let token = TokenDefinition {
        name: jumblebones,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: token clause "Jumblebones can't block" — static, no hook.
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfLeavesBattlefield,
            intervening_if: None,
            effect: leaves_return_ozox,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter {
                        name: reg.interner().lookup("Ozox, the Clattering King"),
                        ..ObjectFilter::default()
                    },
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn leaves_return_ozox(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
