//! Soul of Emancipation — `{4}{G}{W}{U}` 5/7 green-white-blue Avatar. "When
//! this creature enters, destroy up to three other target nonland permanents.
//! For each of those permanents, its controller creates a 3/3 white Angel
//! creature token with flying." ETB trigger targeting up to three nonland
//! permanents; destroy each and give each controller an Angel token.
//! GAP: "for each destroyed permanent, its controller creates a token" — the
//! script helpers provide ids_matching on the battlefield but not "ids of things
//! just destroyed". We can destroy the targets but cannot reference their
//! controllers post-destruction to create tokens. Emit destroy only.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Emancipation");
    let _angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    let avatar = reg.interner_mut().intern("Avatar");
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
            }),
    )
}

fn etb_destroy(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").expect("Angel interned during register()");
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        let TargetChoice::Object(id) = target else { continue };
        let ctrl = script::target_controller(state, *id, trig.controller);
        effects.push(Effect::DestroyPermanent { target: *id });
        // Create 3/3 white Angel with flying for each destroyed permanent's controller.
        let mut angel_subtypes = SubtypeSet::default();
        angel_subtypes.0.insert(angel);
        let token = TokenDefinition {
            name: angel,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: angel_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken { controller: ctrl, token });
    }
    vec![Effect::Sequence(effects)]
}
