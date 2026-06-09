//! Supply Caravan — `{4}{W}` 3/5 white Camel.
//! "When this creature enters, if you control a tapped creature, create a 1/1 white
//! Warrior creature token with vigilance."
//! Intervening-if "if you control a tapped creature" modeled via
//! `conditions::you_control_a` (with a tapped_only filter) on `intervening_if`.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Supply Caravan");
    let camel = reg.interner_mut().intern("Camel");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(camel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // Intervening-if "if you control a tapped creature" via conditions::you_control_a.
                intervening_if: Some(iif_control_tapped_creature),
                effect: etb_warrior_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_control_tapped_creature(state: &GameState, _source: ObjectId, you: PlayerId) -> bool {
    conditions::you_control_a(
        state,
        you,
        &ObjectFilter::new().with_types(TypeLine::CREATURE.into()).tapped_only(),
    )
}

fn etb_warrior_token(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: intervening-if "if you control a tapped creature" not modeled as engine
    // intervening_if — approximating by checking at resolution time
    let tapped_count = {
        use arcana_core::targets::ControllerConstraint;
        arcana_core::script::count_matching(
            state,
            &ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .tapped_only(),
            trig.controller,
        )
    };
    if tapped_count == 0 {
        return Vec::new();
    }
    let warrior = reg.interner().lookup("Warrior")
        .expect("Warrior interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
