//! Anti-Venom, Horrifying Healer — `{W}{W}{W}{W}{W}` 5/5 Legendary
//! Symbiote Hero.
//! "When Anti-Venom enters, if he was cast, return target creature card
//!  from your graveyard to the battlefield.
//!  If damage would be dealt to Anti-Venom, prevent that damage and put
//!  that many +1/+1 counters on him."
//!
//! The ETB reanimation is wired (targeting a creature card in your
//! graveyard). The "if he was cast" intervening-if is not expressible
//! through the demonstrated condition helpers, so it is GAP'd (left None).
//! The damage-prevention-into-counters static is a self-installing
//! replacement effect with an amount-linked counter rider that has no
//! expressible primitive — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Anti-Venom, Horrifying Healer");
    let symbiote = reg.interner_mut().intern("Symbiote");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(symbiote);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP intervening-if: "if he was cast" is not expressible via the
            // demonstrated condition helpers.
            intervening_if: None,
            effect: etb_reanimate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
    // GAP static: "If damage would be dealt to Anti-Venom, prevent that
    // damage and put that many +1/+1 counters on him" — a self-installing
    // damage-replacement with an amount-linked counter rider, no
    // expressible primitive.
}

fn etb_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
