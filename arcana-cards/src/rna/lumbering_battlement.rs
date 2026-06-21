//! Lumbering Battlement — `{4}{W}` 4/5 Beast with Vigilance.
//! "When this creature enters, exile any number of other nontoken creatures
//! you control until it leaves the battlefield. This creature gets +2/+2 for
//! each card exiled with it."

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lumbering Battlement");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP (static): "This creature gets +2/+2 for each card exiled with it" — a
    // dynamic self-pump keyed to the count of cards exiled by THIS object's
    // ExileUntilSourceLeaves linkage; no continuous-effect primitive reads that
    // linkage count.
}

fn etb_exile_creatures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Exile any number of other nontoken creatures you control until it leaves."
    // GAP (fidelity): the player-chosen creatures are exiled, but the "until
    // ~ leaves the battlefield" return linkage is not attached to this minted
    // pick (Effect::ExileUntilSourceLeaves is single-target, not any-number);
    // modeled as a min-0/max-all exile of the chosen nontoken creatures.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Battlefield,
        filter: ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .nontoken(),
        action: PickAction::Exile,
    }]
}
